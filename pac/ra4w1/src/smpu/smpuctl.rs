#[doc = "Register `SMPUCTL` reader"]
pub struct R(crate::R<SMPUCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPUCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPUCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPUCTL` writer"]
pub struct W(crate::W<SMPUCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPUCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMPUCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPUCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OAD` reader - Master Group enable"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Master Group enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non-maskable interrupt."]
    _0 = 0,
    #[doc = "1: Internal reset."]
    _1 = 1,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OAD_A {
        match self.bits {
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
#[doc = "Field `OAD` writer - Master Group enable"]
pub type OAD_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMPUCTL_SPEC, OAD_A, O>;
impl<'a, const O: u8> OAD_W<'a, O> {
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OAD_A::_0)
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OAD_A::_1)
    }
}
#[doc = "Field `PROTECT` reader - Protection of register Protected register SMPUMBIU, SMPUFBIU, SMPUSRAM0, SMPUP0BIU, SMPUP2BIU, SMPUP6BIU,SMPUEXBIU, SMPUEXBIU2"]
pub type PROTECT_R = crate::FieldReader<u8, PROTECT_A>;
#[doc = "Protection of register Protected register SMPUMBIU, SMPUFBIU, SMPUSRAM0, SMPUP0BIU, SMPUP2BIU, SMPUP6BIU,SMPUEXBIU, SMPUEXBIU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROTECT_A {
    #[doc = "0: All Bus Slave register writing is possible."]
    _0 = 0,
    #[doc = "1: All Bus Slave register writing is protected. Read is possible."]
    _1 = 1,
}
impl From<PROTECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as _
    }
}
impl PROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROTECT_A> {
        match self.bits {
            0 => Some(PROTECT_A::_0),
            1 => Some(PROTECT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROTECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROTECT_A::_1
    }
}
#[doc = "Field `PROTECT` writer - Protection of register Protected register SMPUMBIU, SMPUFBIU, SMPUSRAM0, SMPUP0BIU, SMPUP2BIU, SMPUP6BIU,SMPUEXBIU, SMPUEXBIU2"]
pub type PROTECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, SMPUCTL_SPEC, u8, PROTECT_A, 2, O>;
impl<'a, const O: u8> PROTECT_W<'a, O> {
    #[doc = "All Bus Slave register writing is possible."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROTECT_A::_0)
    }
    #[doc = "All Bus Slave register writing is protected. Read is possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROTECT_A::_1)
    }
}
#[doc = "Field `KEY` reader - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
pub type KEY_R = crate::FieldReader<u8, KEY_A>;
#[doc = "Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
    _0X_A5 = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            165 => Some(KEY_A::_0X_A5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X_A5`"]
    #[inline(always)]
    pub fn is_0x_a5(&self) -> bool {
        *self == KEY_A::_0X_A5
    }
}
#[doc = "Field `KEY` writer - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMPUCTL_SPEC, u8, KEY_A, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut W {
        self.variant(KEY_A::_0X_A5)
    }
}
impl R {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Protection of register Protected register SMPUMBIU, SMPUFBIU, SMPUSRAM0, SMPUP0BIU, SMPUP2BIU, SMPUP6BIU,SMPUEXBIU, SMPUEXBIU2"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<0> {
        OAD_W::new(self)
    }
    #[doc = "Bits 1:2 - Protection of register Protected register SMPUMBIU, SMPUFBIU, SMPUSRAM0, SMPUP0BIU, SMPUP2BIU, SMPUP6BIU,SMPUEXBIU, SMPUEXBIU2"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<1> {
        PROTECT_W::new(self)
    }
    #[doc = "Bits 8:15 - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<8> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave MPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpuctl](index.html) module"]
pub struct SMPUCTL_SPEC;
impl crate::RegisterSpec for SMPUCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smpuctl::R](R) reader structure"]
impl crate::Readable for SMPUCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpuctl::W](W) writer structure"]
impl crate::Writable for SMPUCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPUCTL to value 0"]
impl crate::Resettable for SMPUCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
