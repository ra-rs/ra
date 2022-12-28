#[doc = "Register `RWKCNT` reader"]
pub struct R(crate::R<RWKCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWKCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWKCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWKCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWKCNT` writer"]
pub struct W(crate::W<RWKCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWKCNT_SPEC>;
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
impl From<crate::W<RWKCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWKCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAYW` reader - Day-of-Week Counting"]
pub type DAYW_R = crate::FieldReader<u8, DAYW_A>;
#[doc = "Day-of-Week Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAYW_A {
    #[doc = "0: Sunday"]
    _000 = 0,
    #[doc = "1: Monday"]
    _001 = 1,
    #[doc = "2: Tuesday"]
    _010 = 2,
    #[doc = "3: Wednesday"]
    _011 = 3,
    #[doc = "4: Thursday"]
    _100 = 4,
    #[doc = "5: Friday"]
    _101 = 5,
    #[doc = "6: Saturday"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<DAYW_A> for u8 {
    #[inline(always)]
    fn from(variant: DAYW_A) -> Self {
        variant as _
    }
}
impl DAYW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAYW_A {
        match self.bits {
            0 => DAYW_A::_000,
            1 => DAYW_A::_001,
            2 => DAYW_A::_010,
            3 => DAYW_A::_011,
            4 => DAYW_A::_100,
            5 => DAYW_A::_101,
            6 => DAYW_A::_110,
            7 => DAYW_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DAYW_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DAYW_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DAYW_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DAYW_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DAYW_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DAYW_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DAYW_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DAYW_A::_111
    }
}
#[doc = "Field `DAYW` writer - Day-of-Week Counting"]
pub type DAYW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RWKCNT_SPEC, u8, DAYW_A, 3, O>;
impl<'a, const O: u8> DAYW_W<'a, O> {
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DAYW_A::_000)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DAYW_A::_001)
    }
    #[doc = "Tuesday"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DAYW_A::_010)
    }
    #[doc = "Wednesday"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DAYW_A::_011)
    }
    #[doc = "Thursday"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DAYW_A::_100)
    }
    #[doc = "Friday"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DAYW_A::_101)
    }
    #[doc = "Saturday"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DAYW_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DAYW_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Day-of-Week Counting"]
    #[inline(always)]
    pub fn dayw(&self) -> DAYW_R {
        DAYW_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day-of-Week Counting"]
    #[inline(always)]
    #[must_use]
    pub fn dayw(&mut self) -> DAYW_W<0> {
        DAYW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Day-of-Week Counter (in Calendar Count Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwkcnt](index.html) module"]
pub struct RWKCNT_SPEC;
impl crate::RegisterSpec for RWKCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rwkcnt::R](R) reader structure"]
impl crate::Readable for RWKCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwkcnt::W](W) writer structure"]
impl crate::Writable for RWKCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWKCNT to value 0"]
impl crate::Resettable for RWKCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
