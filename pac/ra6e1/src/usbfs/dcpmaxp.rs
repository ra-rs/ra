#[doc = "Register `DCPMAXP` reader"]
pub struct R(crate::R<DCPMAXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCPMAXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCPMAXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCPMAXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCPMAXP` writer"]
pub struct W(crate::W<DCPMAXP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCPMAXP_SPEC>;
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
impl From<crate::W<DCPMAXP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCPMAXP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MXPS` reader - Maximum Packet Size"]
pub type MXPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MXPS` writer - Maximum Packet Size"]
pub type MXPS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DCPMAXP_SPEC, u8, u8, 7, O>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DEVSEL_R = crate::FieldReader<u8, DEVSEL_A>;
#[doc = "Device Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSEL_A {
    #[doc = "0: Address 0000b"]
    _0X0 = 0,
    #[doc = "1: Address 0001b"]
    _0X1 = 1,
    #[doc = "2: Address 0010b"]
    _0X2 = 2,
    #[doc = "3: Address 0011b"]
    _0X3 = 3,
    #[doc = "4: Address 0100b"]
    _0X4 = 4,
    #[doc = "5: Address 0101b"]
    _0X5 = 5,
}
impl From<DEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSEL_A) -> Self {
        variant as _
    }
}
impl DEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVSEL_A> {
        match self.bits {
            0 => Some(DEVSEL_A::_0X0),
            1 => Some(DEVSEL_A::_0X1),
            2 => Some(DEVSEL_A::_0X2),
            3 => Some(DEVSEL_A::_0X3),
            4 => Some(DEVSEL_A::_0X4),
            5 => Some(DEVSEL_A::_0X5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == DEVSEL_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == DEVSEL_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == DEVSEL_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == DEVSEL_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == DEVSEL_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == DEVSEL_A::_0X5
    }
}
#[doc = "Field `DEVSEL` writer - Device Select"]
pub type DEVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DCPMAXP_SPEC, u8, DEVSEL_A, 4, O>;
impl<'a, const O: u8> DEVSEL_W<'a, O> {
    #[doc = "Address 0000b"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0X0)
    }
    #[doc = "Address 0001b"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0X1)
    }
    #[doc = "Address 0010b"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0X2)
    }
    #[doc = "Address 0011b"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0X3)
    }
    #[doc = "Address 0100b"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0X4)
    }
    #[doc = "Address 0101b"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(DEVSEL_A::_0X5)
    }
}
impl R {
    #[doc = "Bits 0:6 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mxps(&mut self) -> MXPS_W<0> {
        MXPS_W::new(self)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    #[must_use]
    pub fn devsel(&mut self) -> DEVSEL_W<12> {
        DEVSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP Maximum Packet Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcpmaxp](index.html) module"]
pub struct DCPMAXP_SPEC;
impl crate::RegisterSpec for DCPMAXP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dcpmaxp::R](R) reader structure"]
impl crate::Readable for DCPMAXP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcpmaxp::W](W) writer structure"]
impl crate::Writable for DCPMAXP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCPMAXP to value 0x40"]
impl crate::Resettable for DCPMAXP_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
