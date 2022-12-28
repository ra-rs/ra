#[doc = "Register `MMPUACEDMAC%s` reader"]
pub struct R(crate::R<MMPUACEDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMPUACEDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMPUACEDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMPUACEDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMPUACEDMAC%s` writer"]
pub struct W(crate::W<MMPUACEDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMPUACEDMAC_SPEC>;
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
impl From<crate::W<MMPUACEDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMPUACEDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Region enable"]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Region enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: EDMAC Region n unit is disabled"]
    _0 = 0,
    #[doc = "1: EDMAC Region n unit is enabled"]
    _1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::_0,
            true => ENABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENABLE_A::_1
    }
}
#[doc = "Field `ENABLE` writer - Region enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUACEDMAC_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "EDMAC Region n unit is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENABLE_A::_0)
    }
    #[doc = "EDMAC Region n unit is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENABLE_A::_1)
    }
}
#[doc = "Field `RP` reader - Read protection"]
pub type RP_R = crate::BitReader<RP_A>;
#[doc = "Read protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_A {
    #[doc = "0: Read permission"]
    _0 = 0,
    #[doc = "1: Read protection"]
    _1 = 1,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        variant as u8 != 0
    }
}
impl RP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::_0,
            true => RP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RP_A::_1
    }
}
#[doc = "Field `RP` writer - Read protection"]
pub type RP_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUACEDMAC_SPEC, RP_A, O>;
impl<'a, const O: u8> RP_W<'a, O> {
    #[doc = "Read permission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RP_A::_0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RP_A::_1)
    }
}
#[doc = "Field `WP` reader - Write protection"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Write permission"]
    _0 = 0,
    #[doc = "1: Write protection"]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Field `WP` writer - Write protection"]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u16, MMPUACEDMAC_SPEC, WP_A, O>;
impl<'a, const O: u8> WP_W<'a, O> {
    #[doc = "Write permission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_A::_0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<1> {
        RP_W::new(self)
    }
    #[doc = "Bit 2 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<2> {
        WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMPU Access Control Register for EDMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmpuacedmac](index.html) module"]
pub struct MMPUACEDMAC_SPEC;
impl crate::RegisterSpec for MMPUACEDMAC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mmpuacedmac::R](R) reader structure"]
impl crate::Readable for MMPUACEDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmpuacedmac::W](W) writer structure"]
impl crate::Writable for MMPUACEDMAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMPUACEDMAC%s to value 0"]
impl crate::Resettable for MMPUACEDMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
