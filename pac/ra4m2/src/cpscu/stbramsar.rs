#[doc = "Register `STBRAMSAR` reader"]
pub struct R(crate::R<STBRAMSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STBRAMSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STBRAMSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STBRAMSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STBRAMSAR` writer"]
pub struct W(crate::W<STBRAMSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STBRAMSAR_SPEC>;
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
impl From<crate::W<STBRAMSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STBRAMSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSBSTBR` reader - Security attributes of each region for Standby RAM"]
pub type NSBSTBR_R = crate::FieldReader<u8, NSBSTBR_A>;
#[doc = "Security attributes of each region for Standby RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSBSTBR_A {
    #[doc = "0: Region7-0 are all Secure."]
    _0X0 = 0,
    #[doc = "1: Region7 is Non-secure. Region6-0 are Secure"]
    _0X1 = 1,
    #[doc = "2: Region7-6 are Non-secure. Region5-0 are Secure."]
    _0X2 = 2,
    #[doc = "3: Region7-5 are Non-secure. Region4-0 are Secure."]
    _0X3 = 3,
    #[doc = "4: Region7-4 are Non-secure. Region 3-0 are Secure."]
    _0X4 = 4,
    #[doc = "5: Region7-3 are Non-secure. Region 2-0 are Secure."]
    _0X5 = 5,
    #[doc = "6: Region7-2 are Non-secure. Region 1-0 are Secure."]
    _0X6 = 6,
    #[doc = "7: Region7-1 are Non-Secure. Region0 is Secure."]
    _0X7 = 7,
}
impl From<NSBSTBR_A> for u8 {
    #[inline(always)]
    fn from(variant: NSBSTBR_A) -> Self {
        variant as _
    }
}
impl NSBSTBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSBSTBR_A> {
        match self.bits {
            0 => Some(NSBSTBR_A::_0X0),
            1 => Some(NSBSTBR_A::_0X1),
            2 => Some(NSBSTBR_A::_0X2),
            3 => Some(NSBSTBR_A::_0X3),
            4 => Some(NSBSTBR_A::_0X4),
            5 => Some(NSBSTBR_A::_0X5),
            6 => Some(NSBSTBR_A::_0X6),
            7 => Some(NSBSTBR_A::_0X7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == NSBSTBR_A::_0X0
    }
    #[doc = "Checks if the value of the field is `_0X1`"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == NSBSTBR_A::_0X1
    }
    #[doc = "Checks if the value of the field is `_0X2`"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == NSBSTBR_A::_0X2
    }
    #[doc = "Checks if the value of the field is `_0X3`"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == NSBSTBR_A::_0X3
    }
    #[doc = "Checks if the value of the field is `_0X4`"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == NSBSTBR_A::_0X4
    }
    #[doc = "Checks if the value of the field is `_0X5`"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == NSBSTBR_A::_0X5
    }
    #[doc = "Checks if the value of the field is `_0X6`"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == NSBSTBR_A::_0X6
    }
    #[doc = "Checks if the value of the field is `_0X7`"]
    #[inline(always)]
    pub fn is_0x7(&self) -> bool {
        *self == NSBSTBR_A::_0X7
    }
}
#[doc = "Field `NSBSTBR` writer - Security attributes of each region for Standby RAM"]
pub type NSBSTBR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STBRAMSAR_SPEC, u8, NSBSTBR_A, 4, O>;
impl<'a, const O: u8> NSBSTBR_W<'a, O> {
    #[doc = "Region7-0 are all Secure."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X0)
    }
    #[doc = "Region7 is Non-secure. Region6-0 are Secure"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X1)
    }
    #[doc = "Region7-6 are Non-secure. Region5-0 are Secure."]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X2)
    }
    #[doc = "Region7-5 are Non-secure. Region4-0 are Secure."]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X3)
    }
    #[doc = "Region7-4 are Non-secure. Region 3-0 are Secure."]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X4)
    }
    #[doc = "Region7-3 are Non-secure. Region 2-0 are Secure."]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X5)
    }
    #[doc = "Region7-2 are Non-secure. Region 1-0 are Secure."]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X6)
    }
    #[doc = "Region7-1 are Non-Secure. Region0 is Secure."]
    #[inline(always)]
    pub fn _0x7(self) -> &'a mut W {
        self.variant(NSBSTBR_A::_0X7)
    }
}
impl R {
    #[doc = "Bits 0:3 - Security attributes of each region for Standby RAM"]
    #[inline(always)]
    pub fn nsbstbr(&self) -> NSBSTBR_R {
        NSBSTBR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Security attributes of each region for Standby RAM"]
    #[inline(always)]
    #[must_use]
    pub fn nsbstbr(&mut self) -> NSBSTBR_W<0> {
        NSBSTBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby RAM memory Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stbramsar](index.html) module"]
pub struct STBRAMSAR_SPEC;
impl crate::RegisterSpec for STBRAMSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stbramsar::R](R) reader structure"]
impl crate::Readable for STBRAMSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stbramsar::W](W) writer structure"]
impl crate::Writable for STBRAMSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STBRAMSAR to value 0xffff_fff0"]
impl crate::Resettable for STBRAMSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_fff0;
}
